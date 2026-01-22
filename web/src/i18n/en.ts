export default {
  common: {
    appName: 'Plain Word',
    loading: 'Loading...',
    save: 'Save',
    cancel: 'Cancel',
    delete: 'Delete',
    edit: 'Edit',
    create: 'Create',
    confirm: 'Confirm',
    back: 'Back',
    clear: 'Clear',
    close: 'Close',
    search: 'Search',
    noData: 'No data',
    success: 'Success',
    error: 'Error',
    required: 'Required',
    optional: 'Optional'
  },
  errors: {
    UNAUTHORIZED: 'Please sign in first',
    INVALID_CREDENTIALS: 'Invalid username or password',
    VALIDATION_ERROR: 'Validation failed',
    NOT_FOUND: 'Resource not found',
    CONFLICT: 'Resource conflict',
    DATABASE_ERROR: 'Database error',
    INTERNAL_ERROR: 'Internal server error',
    UNKNOWN: 'Unknown error'
  },
  nav: {
    home: 'Home',
    wordbooks: 'Wordbooks',
    tags: 'Tags',
    settings: 'Settings',
    login: 'Login',
    register: 'Register',
    logout: 'Logout'
  },
  home: {
    title: 'Plain Word',
    subtitle: 'A beautifully crafted vocabulary notebook that feels like writing on paper. Learn words naturally, your way.',
    getStarted: 'Get Started',
    signIn: 'Sign In',
    features: {
      wordbooks: {
        title: 'Multiple Wordbooks',
        desc: 'Organize your vocabulary by topic, level, or any way you prefer.'
      },
      tags: {
        title: 'Flexible Tags',
        desc: 'Tag words freely and filter them for focused learning sessions.'
      },
      modes: {
        title: 'Study Modes',
        desc: 'Switch between original, translation, or bilingual views.'
      }
    }
  },
  auth: {
    login: {
      title: 'Welcome Back',
      subtitle: 'Sign in to continue learning',
      username: 'Username',
      usernamePlaceholder: 'Enter your username',
      password: 'Password',
      passwordPlaceholder: 'Enter your password',
      submit: 'Sign In',
      submitting: 'Signing in...',
      noAccount: "Don't have an account?",
      signUp: 'Sign up'
    },
    register: {
      title: 'Create Account',
      subtitle: 'Start your vocabulary journey',
      username: 'Username',
      usernamePlaceholder: 'Choose a username',
      email: 'Email',
      emailPlaceholder: 'Enter your email',
      password: 'Password',
      passwordPlaceholder: 'Create a password',
      confirmPassword: 'Confirm Password',
      confirmPasswordPlaceholder: 'Confirm your password',
      submit: 'Create Account',
      submitting: 'Creating account...',
      hasAccount: 'Already have an account?',
      signIn: 'Sign in',
      passwordMismatch: 'Passwords do not match'
    }
  },
  wordbooks: {
    title: 'My Wordbooks',
    new: 'New Wordbook',
    empty: {
      title: 'No wordbooks yet',
      desc: 'Create your first wordbook to start learning',
      action: 'Create Wordbook'
    },
    deleteConfirm: 'Are you sure you want to delete this wordbook?',
    modal: {
      createTitle: 'Create New Wordbook',
      editTitle: 'Edit Wordbook',
      name: 'Name',
      namePlaceholder: 'e.g., TOEFL Vocabulary',
      description: 'Description',
      descriptionPlaceholder: 'Add a description...',
      create: 'Create'
    },
    createdAt: 'Created'
  },
  chapters: {
    title: 'Chapters',
    add: 'Add Chapter',
    backTo: 'Back to',
    empty: {
      title: 'No chapters yet',
      desc: 'Add chapters to organize your vocabulary',
      action: 'Add First Chapter'
    },
    deleteConfirm: 'Are you sure you want to delete this chapter?',
    modal: {
      createTitle: 'Add New Chapter',
      name: 'Chapter Name',
      namePlaceholder: 'e.g., Week 1 - Basic Words',
      create: 'Add Chapter'
    }
  },
  words: {
    title: 'Words',
    add: 'Add Word',
    controls: 'Controls',
    display: 'Display',
    displayModes: {
      original: 'Original',
      translation: 'Translation',
      bilingual: 'Bilingual'
    },
    shuffle: 'Shuffle',
    filterByTag: 'Filter by tag:',
    empty: {
      title: 'No words yet',
      desc: 'Start building your vocabulary',
      action: 'Add First Word'
    },
    filterEmpty: {
      title: 'No matching words',
      desc: 'No words found with the selected tags'
    },
    deleteConfirm: 'Are you sure you want to delete this word?',
    clickToReveal: 'Click to reveal',
    modal: {
      createTitle: 'Add New Word',
      editTitle: 'Edit Word',
      source: 'Source',
      sourcePlaceholder: 'Enter the source text',
      translation: 'Translation',
      translationPlaceholder: 'Enter the translation',
      note: 'Note',
      notePlaceholder: 'Add a note...',
      create: 'Add Word',
      save: 'Save'
    },
    manageTags: 'Manage Tags',
    tags: {
      add: 'Add tag',
      remove: 'Click to remove',
      noMore: 'No more tags available'
    },
    manage: {
      title: 'Manage Words',
      view: 'Study View',
      table: 'Manage View',
      selectAll: 'Select All',
      selected: '{count} selected',
      batchDelete: 'Delete Selected',
      batchEditTags: 'Edit Tags',
      deleteConfirm: 'Are you sure you want to delete {count} selected words?',
      noSelection: 'Please select words first',
      searchPlaceholder: 'Search words...',
      addTag: 'Add Tag',
      removeTag: 'Remove Tag',
      tagEditHint: 'Click to add or remove tags'
    }
  },
  tags: {
    title: 'Tags',
    new: 'New Tag',
    empty: {
      title: 'No tags yet',
      desc: 'Create tags to organize your vocabulary',
      action: 'Create First Tag'
    },
    deleteConfirm: 'Are you sure you want to delete this tag?',
    modal: {
      createTitle: 'Create New Tag',
      name: 'Tag Name',
      namePlaceholder: 'e.g., Important, Review, Easy',
      color: 'Color',
      create: 'Create'
    }
  },
  importExport: {
    importWordbook: 'Import Wordbook',
    importChapter: 'Import Chapter',
    exportWordbook: 'Export Wordbook',
    exportChapter: 'Export Chapter',
    downloadTemplate: 'Download Template',
    selectFile: 'Select File',
    selectFormat: 'Select Format',
    formats: {
      json: 'JSON',
      xml: 'XML',
      xlsx: 'Excel',
      csv: 'CSV'
    },
    name: 'Name',
    namePlaceholder: 'Enter name...',
    import: 'Import',
    importing: 'Importing...',
    success: 'Import Successful',
    chaptersCreated: 'Chapters Created',
    wordsCreated: 'Words Created'
  },
  settings: {
    title: 'Settings',
    account: {
      title: 'Account Information',
      username: 'Username',
      email: 'Email',
      memberSince: 'Member Since'
    },
    display: {
      title: 'Display Settings'
    },
    lineHeight: {
      title: 'Line Height'
    },
    wordSpacing: {
      title: 'Word Spacing'
    },
    about: {
      title: 'About Plain Word',
      desc: 'Plain Word is a beautifully crafted vocabulary notebook application designed to make learning English words feel natural and intuitive, just like writing in a paper notebook.',
      version: 'Version'
    },
    language: {
      title: 'Language',
      current: 'Current Language'
    }
  }
}